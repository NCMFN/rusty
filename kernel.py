# run this once, then reload, and then skip this
!apt install rustc
!gdown --id 1PULtTc-2e9z4bswh_SQqL5oy_4JpfV7c
!chmod +x evcxr_jupyter
!./evcxr_jupyter --install

// install dependency
:dep cmd_lib
use cmd_lib::run_cmd as sh;