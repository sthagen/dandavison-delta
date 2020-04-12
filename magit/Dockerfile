FROM ubuntu:19.10

RUN apt-get update && \
    apt-get install -y \
        build-essential \
        curl \
        emacs \
        git \
        libgl1-mesa-glx \
        mesa-utils \
        texinfo

# Install delta
WORKDIR /tmp
RUN curl -OL https://github.com/dandavison/delta/releases/download/0.0.18/git-delta_0.0.18_amd64.deb && \
    dpkg -i git-delta_0.0.18_amd64.deb && \
    rm git-delta_0.0.18_amd64.deb

# Install magit dependencies
WORKDIR /magit-delta/emacs
COPY magit/install.el .
RUN emacs --batch --load install.el

# Install magit
WORKDIR /magit-delta/
RUN git clone https://github.com/dandavison/magit.git
WORKDIR /magit-delta/magit
RUN git checkout customizable-ansi-color-conversion
RUN git log --stat -n1
ENV BUILD_MAGIT_LIBGIT=""
RUN make

# Install magit-delta
WORKDIR /magit-delta/
RUN git clone https://github.com/dandavison/magit-delta.git

VOLUME /host
WORKDIR /host

CMD emacs \
    --eval "(add-to-list 'load-path \"/magit-delta/magit/lisp\")" \
    --eval "(add-to-list 'load-path \"/magit-delta/magit-delta\")" \
    --eval "(require 'magit-delta)" \
    --eval "(magit-delta-mode +1)" \
    --eval "(magit-status)"
