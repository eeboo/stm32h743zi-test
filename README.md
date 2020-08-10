# Nucleo stm32h743zi `hello world` 

### [OK] SERIAL - virtual com port example
- connect board via usb
- open terminal and start openocd server
```
openocd
```
- open another terminal and start gdb client
```
cargo run --example serial_usart3_ok
(gdb) c
Continuing.
halted: PC: 0x08000e16
```
- open coolterm and check received data from com port
    - Baurate: 19200
    - Stop bit: 1
    - Parity: none

### [KO] SERIAL - uart 4 example
- connect Pin PB9 to logic analyzer
- connect board via usb
- open terminal and start openocd server
```
 openocd
```
- open another terminal and start gdb client
```
cargo run --example serial_usart4_ko
(gdb) c
Continuing.
halted: PC: 0x08000e16
```
4. open logic analyzer and check received data from uart 4
    - Baurate: 19200
    - Stop bit: 1
    - Parity: none

