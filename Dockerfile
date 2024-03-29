# fork of Dockerfile at https://github.com/not-fl3/cargo-quad-apk

FROM archlinux

RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm gcc
RUN pacman -S --noconfirm jdk8-openjdk unzip wget cmake rustup openssl pkgconf

# github override HOME, so here we are
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH 

RUN rustup toolchain install 1.77.0
RUN rustup default 1.77
RUN rustc --version

RUN rustup target add armv7-linux-androideabi
RUN rustup target add aarch64-linux-android
RUN rustup target add i686-linux-android
RUN rustup target add x86_64-linux-android

# Install Android SDK
ENV ANDROID_HOME /opt/android-sdk-linux
ENV JAVA_HOME /usr/lib/jvm/default
RUN mkdir ${ANDROID_HOME} && \
    cd ${ANDROID_HOME} && \
    wget -q https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip && \
    unzip -q sdk-tools-linux-4333796.zip && \
    rm sdk-tools-linux-4333796.zip && \
    chown -R root:root /opt
RUN mkdir -p ~/.android && touch ~/.android/repositories.cfg
RUN yes | ${ANDROID_HOME}/tools/bin/sdkmanager "platform-tools" | grep -v = || true
RUN yes | ${ANDROID_HOME}/tools/bin/sdkmanager "platforms;android-31" | grep -v = || true
RUN yes | ${ANDROID_HOME}/tools/bin/sdkmanager "build-tools;31.0.0"  | grep -v = || true
RUN ${ANDROID_HOME}/tools/bin/sdkmanager --update | grep -v = || true

# Install Android NDK
RUN cd /usr/local && \
    wget -q http://dl.google.com/android/repository/android-ndk-r25-linux.zip && \
    unzip -q android-ndk-r25-linux.zip && \
    rm android-ndk-r25-linux.zip
ENV NDK_HOME /usr/local/android-ndk-r25

# Make directory for user code
RUN mkdir /root/src

# Copy contents to container. Should only use this on a clean directory
COPY . /root/src/

# Install mobile build
RUN cargo install --git https://github.com/tauri-apps/cargo-mobile2

# Add build-tools to PATH, for apksigner
ENV PATH="/opt/android-sdk-linux/build-tools/31.0.0/:${PATH}"

WORKDIR /root/src