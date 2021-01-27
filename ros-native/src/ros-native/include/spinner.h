#ifndef ROS_ONE_SPINNER_H_
#define ROS_ONE_SPINNER_H_

/*
 * PATH: /opt/ros/melodic/include/ros/spinner.h
 */

#ifdef __cplusplus
extern "C" {
#endif

#include <inttypes.h>

#include "macros.h"
#include "callback_queue.h"

/**
 * \brief Spinner which spins in a single thread.
 */
typedef struct single_threaded_spinner single_threaded_spinner;

single_threaded_spinner *single_threaded_spinner_create();

void single_threaded_spinner_destroy(single_threaded_spinner *sp);

void single_threaded_spinner_spin(single_threaded_spinner *sp, callback_queue *cq);

/**
 * \brief Spinner which spins in multiple threads.
 */
typedef struct multi_threaded_spinner multi_threaded_spinner;

/**
 * \param thread_count Number of threads to use for calling callbacks.   0 will
 * automatically use however many hardware threads exist on your system.
 */
multi_threaded_spinner *multi_threaded_spinner_create(uint32_t thread_count);

void multi_threaded_spinner_destroy(multi_threaded_spinner *sp);

void multi_threaded_spinner_spin(multi_threaded_spinner *sp, callback_queue *cq);

/**
 * \brief AsyncSpinner is a spinner that does not conform to the abstract Spinner interface.  Instead,
 * it spins asynchronously when you call start(), and stops when either you call stop(), ros::shutdown()
 * is called, or its destructor is called
 *
 * AsyncSpinner is reference counted internally, so if you copy one it will continue spinning until all
 * copies have destructed (or stop() has been called on one of them)
 */
typedef struct async_spinner async_spinner;

async_spinner *async_spinner_create(uint32_t thread_count);
async_spinner *async_spinner_create_with_queue(uint32_t thread_count, callback_queue *cq);

void async_spinner_destroy(async_spinner *sp);

/**
 * \brief Start this spinner spinning asynchronously
 */
void async_spinner_start(async_spinner *sp);

/**
 * \brief Stop this spinner from running
 */
void async_spinner_stop(async_spinner *sp);

#ifdef __cplusplus
}
#endif

#endif //ROS_ONE_SPINNER_H_