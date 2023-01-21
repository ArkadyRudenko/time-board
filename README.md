# Time-board app

You can manage your time by creating a project and tasks for your work

### HTTP API:
    - registration: curl -X POST "http://localhost:8000/api-v1/registration"  -H 'Content-Type: application/json' -d '{
                                         "username": "arkady",
                                         "login": "arkady@mail.ru",
                                         "password": "qwerty12"}'
    - login:        curl -X POST "http://localhost:8000/api-v1/login"  -H 'Content-Type: application/json' -d '{
                                                                  "login": "arkady@mail.ru",
                                                                  "password": "qwerty12"}'
     Response:      {"token":"YlDl24XpA17iq91MA+3MBLtJ5goiJWS6vf8bapRp69M="}

    - create project: curl -X POST "http://localhost:8000/api-v1/project"  -H 'Content-Type: application/json' -d '{
                                           "title": "first project",
                                           "description": "first description",
                                           "access_token": "YlDl24XpA17iq91MA+3MBLtJ5goiJWS6vf8bapRp69M="
                                            }'
     Response:        "Project was added"
