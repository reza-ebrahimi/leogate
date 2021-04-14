mod core;
mod ros_msgs;
mod ros_schema;
mod sys_schema;

mod main_ros;
mod main_sys;

fn main() -> std::io::Result<()> {
  let _ros_thread_handle = std::thread::spawn(move || {
    main_ros::main().unwrap();
  });

  main_sys::main()
}
