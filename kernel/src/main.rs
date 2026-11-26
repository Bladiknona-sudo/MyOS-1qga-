 > 0 {
 console.input[console.input_len] = ch;
 console.input_len += 1;
                            }
                        }
                    }
                }
            } else {
                unsafe { asm!("pause", options( omem, nostack, preserves_flags)); }
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    serial_write("MYOS_PANIC ");
    halt()
}
