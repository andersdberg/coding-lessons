# Docker

## Introduction
We are going to jump right into the deep end. Docker is an important tool for modern software development **and** for the rest of this course. 

At the time of writing, the [Linux operating system](https://www.linux.com/what-is-linux/) is used to power over 90% of the worlds web servers.

We will have future in-depth lessons about Linux. However, it is important to understadn what Docker does and what problems it solves. In the beginning of software development, developers would struggle with sharing code with other developers. It was common for a piece of code to run correctly on one persons machine, and not run at all on another. The were many reasons for this, but the most common were that they may have been on different versions of the same operating system, different operating systems entirely, had different versions of dependent software installed, etc. A common phrase back in the day was, "It works on machine". There have been several solutions to this problem over the years, but Docker is the most modern and widespread solution.

Docker allows you to define an entire operating system and it's dependencies in a single file that can be executed with a single command. This means that developers all over the world, using different operating systems, can expect the code to run everywhere. Using Docker for this course ensures that everyone can reliably participate regardless of what machine they are on, and helps learn another important tool for professional development.

## How to use Docker

First, you must [dowload Docker](https://docs.docker.com/get-docker/).

Next, once Docker is downloaded, within your terminal, run `docker run hello-world`. This will pull (download) the image, execute a command, and display the output in your terminal. After reading the output, try running the command again and notice how much faster it executes. This is because the Docker image is already on your machine; the most time consuming part of using a Docker image is downloading it. Once it is on your machine, execution is virtually instant.

The "hello world" example is very simple. It executed a script pre-defined inside of the Docker container and exited. This example does not allow you to modify or interact with the underlying operating system. 

In the real world of software development, you will be executing code _inside_ of a docker container. For example, if you wanted to execute your code in a controlled environment, you can attach the code file to the container as illustrated below. The following command includes a `--volume` argument that tell the docker container that you want to "mount" a file from your personal machine into the operating system inside the Docker container. In this example, you're mounting the `hello-world.sh` shell script from the `lesson-1/hack/docker` directory to the root directory of the Ubuntu operating system inside of the Docker container, `/hello-world.sh`. The `ubuntu` line in the following example tells Docker which image to run locally. This will need to be downloaded ("pulled" in developer lingo) the first time that you run this command. The `/hello-world.sh` line of this command tells Docker which command to run, and in this case it is executing the `hello-world.sh` script at the `/` root directory. Feel free to edit the `echo....` command within the `lesson-1/hack/docker/hello-world.sh` file, and run this command again to visualize the changes.

Notes: the value on the left of the colon `:` is the path to the file on your local machine. The value on the right, is where to mount that file inside of the Docker container. The `$(pwd)` expands to the value of `pwd` or your present working directory, as a shortcut for typing it all out.

Note: need to explain root directory in terminal section.

```
docker run \
    --volume "$(pwd)/hack/docker/hello-world.sh:/hello-world.sh" \
    ubuntu \
    /hello-world.sh
```

This is how you execute your code, on your machine, in a controlled linux environment. This is the most reliable way to execute code to ensure that it will work in it's intended production environment, which is typically a linux machine in the cloud.

TODO: Maybe a CLI inside a container is smarter to show first
The following command runs the `ubuntu` image and executes the bash command
```
docker run --interactive --tty --rm ubuntu bash
```

^^ run `pwd` to confirm that you are in the root directory `/`, and run  `ls -al` to see the file structure inside of the Ubuntu operating system. Compare this to your local machine - you will see that while inside of this Ubuntu machine, you are in a sandbox environment. This is a completely separate environment. Anything you do in this machine will not affect your personal computer. 

if you mount a file into the docker container, then that file will be synced across both local machine and the virtual Ubuntu opoerating system. If you were to edit the file within the container, the edits would appear on your local machine.
```
docker run --interactive --tty --rm --volume "$(pwd)/hack/docker/hello-world.sh:/hello-world.sh" ubuntu bash
```

Run a website:

```
docker run -p 8080:80 \
    -v "$(pwd)/hack/docker/website.html:/usr/share/nginx/html/index.html:ro" \
    -d nginx
```

```
# compile the code
docker run --entrypoint="" --workdir="/docker" -v "$(pwd)/hack/docker:/docker" rust:1.5.0 rustc hello.rs

# run the code
docker run --entrypoint="" --workdir="/docker" -v "$(pwd)/hack/docker:/docker" rust:1.5.0 ./hello
```

## Activities

- Run a simple dockerfile such as hello-world
- Run a dockerfile with a mounted stdin tty to use at your terminal
- Run a dockerfile with a mounted file or directory from your local machine
- Run a dockerfile with a webserver

## Notes

- A dependency is a third-party library that is used to solve a well-known problem. For example, it is not necessary to write code that connects to a database, as there are already many libraries that accomplish this task. Simply, download the dependency and use their library in your code and save yourself time.
- An example of computers running different versions of software might be something like different versions of the same operating system, Zoom, Minecraft, etc.