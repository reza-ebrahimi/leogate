#ifndef ROS_ONE_CALLBACK_QUEUE_H_
#define ROS_ONE_CALLBACK_QUEUE_H_

/*
 * PATH: /opt/ros/melodic/include/ros/callback_queue.h
 */

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>

typedef struct callback_queue callback_queue;

typedef enum CallOneResult {
  Called,
  TryAgain,
  Disabled,
  Empty,
} CallOneResult;

callback_queue *callback_queue_create(bool enabled);

void callback_queue_destroy(callback_queue *cq);

/**
 * \brief Pop a single callback off the front of the queue and invoke it.  If the callback was not ready to be called,
 * pushes it back onto the queue.
 */
CallOneResult callback_queue_callOne(callback_queue *cq);

/**
 * \brief Invoke all callbacks currently in the queue.  If a callback was not ready to be called, pushes it back onto the queue.
 */
void callback_queue_callAvailable(callback_queue *cq);

/**
 * \brief returns whether or not the queue is empty
 */
bool callback_queue_empty(callback_queue *cq);

/**
 * \brief returns whether or not the queue is empty
 */
bool callback_queue_isEmpty(callback_queue *cq);

/**
 * \brief Removes all callbacks from the queue.  Does \b not wait for calls currently in progress to finish.
 */
void callback_queue_clear(callback_queue *cq);

/**
 * \brief Enable the queue (queue is enabled by default)
 */
void callback_queue_enable(callback_queue *cq);

/**
 * \brief Disable the queue, meaning any calls to addCallback() will have no effect
 */
void callback_queue_disable(callback_queue *cq);

/**
 * \brief Returns whether or not this queue is enabled
 */
bool callback_queue_isEnabled(callback_queue *cq);

#ifdef __cplusplus
}
#endif

#endif //ROS_ONE_CALLBACK_QUEUE_H_