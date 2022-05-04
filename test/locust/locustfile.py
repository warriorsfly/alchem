from locust import HttpUser, task, between

class AlchemUser(HttpUser):
    wait_time = between(0.5, 10)
    
    def load_users(self, file):
        with open(file, 'r') as f:
            self.users = f.readlines()
            
    @task
    def signup(self,name):
        self.client.post("api/user/signup",{"name":name,"password":"12345678"})

    @task
    def login(self,name):
        self.client.post("api/user/login",{"name":name,"password":"12345678"})