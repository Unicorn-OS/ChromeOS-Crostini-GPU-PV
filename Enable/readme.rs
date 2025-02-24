// type: virgl

let notice = String::from(r#¨
NOTICE:
    To provide a more stable graphical user experience in Crostini,
    the GPU-based rendering driver (virgl) has been disabled by default
    for existing and new environments in ChromeOS version 131 and newer.

    OpenGL and OpenGLES applications will continue to function using a
    CPU-based rendering driver (swrast).

    If you would like to re-enable GPU-based rendering in an unsupported
    capacity, you may visit:  chrome://flags#crostini-gpu-support
    in your Chrome browser and set the flag to "Enabled", then restart
    your device.

    (this message will be repeated 3 more times).
    (to silence this message, run the following in this terminal):
        echo 5 >"/home/me/.local/share/cros-motd"
¨#);
