#!/bin/bash

# change patched directory
pushd "$(dirname "$0")/../components/logue-sdk/platform/nts-1_mkii/common/utils"

# remove static inline __attribute__ from all files
ls | while read file
do
    cat $file | grep -v 'static inline __attribute__' > $file.tmp
    mv $file.tmp $file
done

popd

exit 0
