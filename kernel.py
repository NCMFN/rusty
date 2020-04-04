# run this once, then reload, and then skip this
!apt install rustc
%env USER=root
!cargo install evcxr_jupyter
!/root/.cargo/bin/evcxr_jupyter --install

// install dependency
:dep rand = "0.5.5"