#Worker app
Consumes tasks from the task queue.

##Localhost
Install RabbitMQ or AMQP compliant broker:
[https://wiki.archlinux.org/title/RabbitMQ](https://wiki.archlinux.org/title/RabbitMQ)

**Run the AMQP server:**
`sudo systemctl enable rabbitmq.service`

`sudo systemctl start rabbitmq.service`

`sudo systemctl status rabbitmq.service`

**To use the admin page:**

``

Then go to [http://localhost:15672/](http://localhost:15672/)
on localhost use:
username `guest`
password: `guest`
