# Compile latest nsjail from source.
#
FROM alpine:3.13 AS nsjail

ENV BUILD_DEPENDENCIES build-base linux-headers bsd-compat-headers git protobuf-dev bison flex \
                       pkgconfig libnl3-dev

RUN apk add --no-cache $BUILD_DEPENDENCIES \
    && git clone --single-branch --depth 1 https://github.com/google/nsjail.git /nsjail \
    && cd /nsjail && make -j && cp nsjail /usr/sbin/ \
    && rm -R /nsjail \
    && apk del --purge $BUILD_DEPENDENCIES

# Set up the challenge image using nsjail.
#
FROM alpine:3.13

RUN apk add --no-cache libstdc++ libnl3 protobuf socat

RUN adduser --no-create-home --uid 1337 pctf

COPY --from=nsjail /usr/sbin/nsjail /usr/sbin/nsjail
