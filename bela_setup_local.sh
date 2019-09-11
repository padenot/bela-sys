#!/bin/sh -xe

# OSX and Linux only for now

BELA=bela.local
DIRECTORY="arm-bela-linux-gnueabihf"

# initial setup: download and extract an arm toolchain locally, and pull some
# files from Bela
if [ ! -d $DIRECTORY ]
then
  host=`uname`
  if [ "$host" = "Darwin" ]; then
    PACKAGE="arm-bela-linux-gnueabihf.zip"
    curl -o $PACKAGE http://files.bela.io/gcc/$PACKAGE
    unzip $PACKAGE
    rm $PACKAGE
  elif [ "$host" = "Linux" ]; then
    PACKAGE="gcc-linaro-6.3.1-2017.02-x86_64_arm-linux-gnueabihf.tar.xz"
    wget https://releases.linaro.org/components/toolchain/binaries/6.3-2017.02/arm-linux-gnueabihf/$PACKAGE
    tar xf $PACKAGE
    mv gcc-linaro-6.3.1-2017.02-x86_64_arm-linux-gnueabihf $DIRECTORY
    rm -r $PACKAGE
  fi

  scp -r root@$BELA:/root/Bela/include .
  scp -r root@$BELA:/root/Bela/lib .
  scp root@bela.local:/usr/local/lib/libprussdrv.so lib
  scp root@bela.local:/usr/xenomai/lib/libmodechk.so lib
  scp root@bela.local:/usr/xenomai/lib/libcobalt.so lib
  scp root@bela.local:/lib/arm-linux-gnueabihf/librt.so.1 lib
fi

export PATH=$PATH:`pwd`/$DIRECTORY/bin
