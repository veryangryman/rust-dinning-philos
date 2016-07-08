# rust-dinning-philos

Complete example "Dinning philosophers" from the rust book.
I don't use the other-hand way to avoid deadlock.
Every for under the mutex.
Every philosopher takes a left fork first, then takes a right fork.
Every philosopher implement this flow:

1. Try to take a fork in left hand (try_lock(left_fork)).
  * On failure (fork is busy - locked by other) - go to step 6.
1. Try to take a fork in other hand (try_lock(right_fork))
  * On failure - drop the left fork and to to step 6.
1. Eat (at this step philo has both forks) - wait a timeout (speed)
1. Drop forks
1. Finish (break the loop)
1. Wait the patience timeout and to to step 1.
