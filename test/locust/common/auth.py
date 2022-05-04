import requests

class JwtAuth(requests.auth.AuthBearer):
    """
    JWT Auth
    """
    def __init__(self, token):
        self.token = token

    def __call__(self, r):
        r.headers['Authorization'] = 'Bearer {}'.format(self.token)
        return r