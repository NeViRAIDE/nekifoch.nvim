cargo b && mkdir lua -p && mv target/debug/libnekifoch.so lua/nekifoch.so -fn
# nvim
set_rtp=":set rtp+=$PWD"
cmd="
:lua require'nekifoch'
"
RUST_BACKTRACE=1 nvim -u NONE --headless +"$set_rtp" +"$cmd" +quit
