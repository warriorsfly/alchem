from locust import FastHttpUser,TaskSet,task

class WebsocketUser(FastHttpUser):
    wait_time = 0
    host = "ws://localhost:3000/ws"
    def on_start(self):
        self.client.ws.connect(self.host)

    def on_stop(self):
        self.client.ws.close()

    class Tasks(TaskSet):
        @task
        def send_message(self):
            self.client.ws.send_message("Hello World")
            self.client.ws.receive_message()