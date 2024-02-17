BINARYEN_VERSION=${1}
BINARYEN_URL="https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz"

rm -rf ./libs

mkdir ./libs

# linux
mkdir ./libs/linux
wget -O ./libs/linux/binaryen.tar.gz ${BINARYEN_LINUX_URL}
tar -zxvf ./libs/linux/binaryen.tar.gz -C ./libs/linux --strip-components 1
mv ./libs/linux/include ./libs
rm -rf ./libs/linux
