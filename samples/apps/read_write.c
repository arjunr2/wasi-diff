#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <sys/uio.h>
#include <unistd.h>
#include <string.h>
#include <errno.h>

void copy_file_read_write_vectored(const char *src_path, const char *dst_path) {
    // --- Step 1: Read from source file ---
    int src_fd = open(src_path, O_RDONLY);
    if (src_fd < 0) {
        perror("open source");
        return;
    }

    char rbuf1[1024] = {0};
    char rbuf2[1024] = {0};
    struct iovec iov_read[2] = {
        { .iov_base = rbuf1, .iov_len = sizeof(rbuf1) },
        { .iov_base = rbuf2, .iov_len = sizeof(rbuf2) }
    };

    ssize_t bytes_read = readv(src_fd, iov_read, 2);
    if (bytes_read < 0) {
        perror("readv source");
        close(src_fd);
        return;
    }
    close(src_fd);
    printf("[readv src] Read %zd bytes\n", bytes_read);

    // --- Step 2: Write to destination file ---
    int dst_fd = open(dst_path, O_WRONLY | O_CREAT | O_TRUNC, 0644);
    if (dst_fd < 0) {
        perror("open dest");
        return;
    }

    struct iovec iov_write[2] = {
        { .iov_base = rbuf1, .iov_len = (bytes_read > sizeof(rbuf1)) ? sizeof(rbuf1) : bytes_read },
        { .iov_base = rbuf2, .iov_len = (bytes_read > sizeof(rbuf1)) ? bytes_read - sizeof(rbuf1) : 0 }
    };

    ssize_t bytes_written = writev(dst_fd, iov_write, 2);
    if (bytes_written < 0) {
        perror("writev dest");
        close(dst_fd);
        return;
    }
    close(dst_fd);
    printf("[writev dst] Wrote %zd bytes\n", bytes_written);

    // --- Step 3: Read back from destination file ---
    dst_fd = open(dst_path, O_RDONLY);
    if (dst_fd < 0) {
        perror("reopen dest");
        return;
    }

    char verify1[1024] = {0};
    char verify2[1024] = {0};
    struct iovec iov_verify[2] = {
        { .iov_base = verify1, .iov_len = sizeof(verify1) },
        { .iov_base = verify2, .iov_len = sizeof(verify2) }
    };

    ssize_t bytes_verified = readv(dst_fd, iov_verify, 2);
    if (bytes_verified < 0) {
        perror("readv verify");
        close(dst_fd);
        return;
    }
    close(dst_fd);
    printf("[readv dst] Verified %zd bytes\n", bytes_verified);
}


int main(int argc, char *argv[]) {
  if (argc != 3) {
    printf("Invalid arguments; expects two file paths (read file and write file); got %d\n", argc - 1);
    return 1;
  }
  copy_file_read_write_vectored(argv[1], argv[2]);
  return 0;

}

