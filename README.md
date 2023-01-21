# Time-board app

You can manage your time by creating a project and tasks for your work

### HTTP API:
    - registration: curl -X POST "http://localhost:8000/api-v1/registration"  -H 'Content-Type: application/json' -d '{
                                                                  "username": "arkady",
                                                                  "login": "arkady@mail.ru",
                                                                  "password": "qwerty12"}'
---

    - login:        curl -X POST "http://localhost:8000/api-v1/login"  -H 'Content-Type: application/json' -d '{
                                                                  "login": "arkady@mail.ru",
                                                                  "password": "qwerty12"}'
     Response:      {"token":"YlDl24XpA17iq91MA+3MBLtJ5goiJWS6vf8bapRp69M="}
---
    - create project: curl -X POST "http://localhost:8000/api-v1/project?access_token=YlDl24XpA17iq91MA+3MBLtJ5goiJWS6vf8bapRp69M="  -H 'Content-Type: application/json' -d '{
                                           "title": "first project",
                                           "description": "first description"
                                            }'
     Response:        "Project was added"
---
    - get all projects: curl -X GET "http://localhost:8000/api-v1/projects?access_token=YlDl24XpA17iq91MA+3MBLtJ5goiJWS6vf8bapRp69M="
---
    - create new task with project_id: curl -X POST "http://localhost:8000/api-v1/project/<project_id>/task?access_token=<access_token>"  -H 'Content-Type: application/json'
                            -d '{ "description": "first task description"}'

     Response: "Task was added"
---
    get_all_task: curl -X GET "http://localhost:8000/api-v1/project/<project_id>/tasks?access_token=<access_token>"

     Response: [{"id":"e12160c3-424b-4448-a3e3-d4f83cf9caf0",
                "description":"first task description",
                "project_id":"f6c23b88-089a-4a92-ae34-ec416d05c667"}]
---
    start session with task: curl -X POST "http://localhost:8000/api-v1/project/<project_id>/task/<task_id>?access_token=<access_token>"

    Response: {"id":"01c79806-a7dc-488a-8d54-baa2bea25935","task_id":"e12160c3-424b-4448-a3e3-d4f83cf9caf0",
                "start_task":{"secs_since_epoch":1674292483,"nanos_since_epoch":876101000},
                "end_task":{"secs_since_epoch":1674292483,"nanos_since_epoch":876101000}}
---
    end session with task: curl -X POST "http://localhost:8000/api-v1/project/<project_id>/task/<task_id>/<session_id>?access_token=<access_token

    Response: "Session ended"
---
    get all time with project: curl -X GET "http://localhost:8000/api-v1/project/<project_id>/time?access_token=<access_token>

    Response: {"secs":124,"nanos":532509115}