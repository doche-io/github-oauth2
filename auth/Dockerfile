FROM archlinux:latest
RUN pacman --noconfirm -Syyyu
COPY ./target/release/auth /root/auth
CMD /root/auth
