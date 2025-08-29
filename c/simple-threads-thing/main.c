#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <errno.h>
#include <error.h>
#include <pthread.h>
#include <stdio.h>
#include <time.h>
#include <unistd.h>

#define THREADS_NUMBER 4
#define ITERATIONS     10

static struct timespec WAIT_TIME = {1 /*seconds*/,
                                    0 /*nanoseconds*/};

typedef struct _thread {
    pthread_t thread;
    int       assigned_id;
    int       tid;

    void *(*thread_handler)(void *);
} Thread;

void *sum_routine(void *_thread_struct) {
    unsigned int i, sum = 0;
    Thread      *current_thread = NULL;
    int          current_tid    = gettid();

    if (_thread_struct) {
        current_thread      = (Thread *)_thread_struct;
        current_thread->tid = current_tid;

        printf("[pid: %d | tid: %d] New thread spwaned with "
            "assigned_id = %d \n",
            getpid(),
            gettid(),
            current_thread->assigned_id
        );
    } else {
        printf("[pid: %d | tid: %d] Thread struct needs to be "
            "provided as argument\n",
            getpid(),
            current_thread->tid
        );
        pthread_exit(NULL);
    }

    for (i = 1; i <= ITERATIONS; ++i) {
        nanosleep(&WAIT_TIME, NULL);

        printf("[pid: %d | tid: %d] Thread %d -> Integer sum up "
            "to %d = %d\n",
            getpid(),
            gettid(),
            current_thread->assigned_id,
            i,
            (sum += i)
        );

    }

    pthread_exit(NULL);
}

void *factorial_routine(void *_thread_struct) {
    unsigned int i, prod = 1;
    Thread      *current_thread = NULL;
    int          current_tid    = gettid();

    if (_thread_struct) {
        current_thread      = (Thread *)_thread_struct;
        current_thread->tid = current_tid;

        printf("[pid: %d | tid: %d] New thread spwaned with "
            "assigned_id = %d \n",
            getpid(),
            gettid(),
            current_thread->assigned_id
        );
    } else {
        printf("[pid: %d | tid: %d] Thread struct needs to be "
            "provided as argument\n",
            getpid(),
            current_thread->tid
        );
        pthread_exit(NULL);
    }

    for (i = 1; i <= ITERATIONS; ++i) {
        nanosleep(&WAIT_TIME, NULL);
        printf("[pid: %d | tid: %d] Thread %d -> Factorial of "
            "%d = %d\n",
            getpid(),
            gettid(),
            current_thread->assigned_id,
            i,
            (prod *= i)
        );
    }
    pthread_exit(NULL);
}

void *pow_routine(void *_thread_struct) {
    unsigned int i, pow = 1;
    Thread      *current_thread = NULL;
    int          current_tid    = gettid();

    if (_thread_struct) {
        current_thread      = (Thread *)_thread_struct;
        current_thread->tid = current_tid;

        printf("[pid: %d | tid: %d] New thread spwaned with "
            "assigned_id = %d \n",
            getpid(),
            gettid(),
            current_thread->assigned_id
        );
    } else {
        printf("[pid: %d | tid: %d] Thread struct needs to "
            "be provided as argument\n",
            getpid(),
            current_thread->tid
        );
        pthread_exit(NULL);
    }

    for (i = 1; i <= ITERATIONS; ++i) {
        nanosleep(&WAIT_TIME, NULL);
        printf("[pid: %d | tid: %d] Thread %d -> 2 to the power "
            "%d = %d\n",
            getpid(),
            gettid(),
            current_thread->assigned_id,
            i,
            (pow = 1 << i)
        );
    }
    pthread_exit(NULL);
}

void *fib_routine(void *_thread_struct) {
    Thread      *current_thread = NULL;
    unsigned int i, a = 0, b = 1;
    int          current_tid = gettid();

    if (_thread_struct) {
        current_thread      = (Thread *)_thread_struct;
        current_thread->tid = current_tid;

        printf("[pid: %d | tid: %d] New thread spwaned with "
            "assigned_id = %d \n",
            getpid(),
            current_thread->tid,
            current_thread->assigned_id);
    } else {
        printf("[pid: %d | tid: %d] Thread struct needs to "
            "be provided as argument\n",
            getpid(),
            current_tid
        );
        pthread_exit(NULL);
    }

    for (i = 1; i <= ITERATIONS; ++i) {
        nanosleep(&WAIT_TIME, NULL);
        printf("[pid: %d | tid: %d] Thread %d -> Fibonacci #%d "
            "= %d\n",
            getpid(),
            gettid(),
            current_thread->assigned_id,
            i,
            (a += b)
        );

        a = a ^ b;
        b = a ^ b;
        a = a ^ b;
    }
    pthread_exit(NULL);
}

int main(void) {
    int    i, errv = 0;
    Thread threads[THREADS_NUMBER];

    printf("[pid: %d | tid: %d] Main process thread\n",
        getpid(),
        gettid()
    );

    threads[0].thread_handler = sum_routine;
    threads[1].thread_handler = factorial_routine;
    threads[2].thread_handler = pow_routine;
    threads[3].thread_handler = fib_routine;

    for (i = 0; i < THREADS_NUMBER; ++i) {
        threads[i].assigned_id = i + 1;

        pthread_create(&threads[i].thread,
            NULL,
            threads[i].thread_handler,
            &threads[i]
        );
        if ((errv = errno)) perror("Error creating thread");
    }

    for (i = 0; i < THREADS_NUMBER; ++i) {
        pthread_join(threads[i].thread, 0);
        printf("Thread #%d [tid: %d] joined main thread %d\n",
            threads[i].assigned_id,
            threads[i].tid, gettid()
        );
    }
    return 0;
}
