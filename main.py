from jupyter_client.manager import KernelManager
import zmq
import threading

km = KernelManager()
km.start_kernel()

kc = km.client()

code = "2 + 2"
msg_id = kc.execute(code)

print(msg_id)
print()
print()


def print_zmq_messages():
    km = KernelManager()
    km.start_kernel()
    kc = km.client()

    code = "2 + 2"
    msg_id = kc.execute(code)

    shell_socket = kc.shell_channel
    iopub_socket = kc.iopub_channel

    context = zmq.Context.instance()
    poller = zmq.Poller()
    poller.register(shell_socket.socket, zmq.POLLIN)
    poller.register(iopub_socket.socket, zmq.POLLIN)

    while True:
        sockets = dict(poller.poll())
        if (
            shell_socket.socket in sockets
            and sockets[shell_socket.socket] == zmq.POLLIN
        ):
            msg = shell_socket.get_msg()
            print("SHELL:", msg)

        if (
            iopub_socket.socket in sockets
            and sockets[iopub_socket.socket] == zmq.POLLIN
        ):
            msg = iopub_socket.get_msg()
            print("IOPUB:", msg)


threading.Thread(target=print_zmq_messages).start()

while 1 == 1:
    print("new iteration")
    try:
        kc_msg = kc.get_iopub_msg(timeout=3)
        print("iopub received")
        # print(kc_msg)
        # print()
        # print()

    except:
        print("Timeout")
        break
