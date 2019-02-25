FROM japaric/armv7-unknown-linux-musleabihf:v0.1.14

RUN dpkg --add-architecture arm64
RUN echo "deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports trusty-updates main universe" >> /etc/apt/sources.list
RUN apt update; exit 0
RUN apt install libudev-dev:arm64