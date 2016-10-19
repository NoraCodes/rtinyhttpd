# Building the Legacy Server

In order to make the game server build, I had to follow the author's instructions:

```
/* This program compiles for Sparc Solaris 2.6.
 * To compile for Linux:
 *  1) Comment out the #include <pthread.h> line.
 *  2) Comment out the line that defines the variable newthread.
 *  3) Comment out the two lines that run pthread_create().
 *  4) Uncomment the line that runs accept_request().
 *  5) Remove -lsocket from the Makefile.
 */
```

However, these instructions seemed to be out of date; there are not two occurances
of `pthread_create`, and the server does not work with these modifications.
