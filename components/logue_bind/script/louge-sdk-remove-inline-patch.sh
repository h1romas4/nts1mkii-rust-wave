#!/bin/bash

# HOME
SCRIPT_HOME=$(dirname "$0")

# remove static inline __attribute__ from all util files
pushd "${SCRIPT_HOME}/../../logue-sdk/platform/nts-1_mkii/common/utils"
ls | while read file
do
    cat $file | grep -v 'static inline __attribute__' > $file.tmp
    mv $file.tmp $file
done
popd

# remove static fast_inline
sed -i 's/static fast_inline//g' ${SCRIPT_HOME}/../../logue-sdk/platform/nts-1_mkii/common/osc_api.h
sed -i 's/static fast_inline//g' ${SCRIPT_HOME}/../../logue-sdk/platform/nts-1_mkii/common/fx_api.h
sed -i 's/static inline __attribute__((always_inline, optimize("Ofast")))//g' ${SCRIPT_HOME}/../../logue-sdk/platform/nts-1_mkii/common/fx_api.h
sed -i 's/static inline __attribute__((always_inline, optimize("Ofast")))//g' ${SCRIPT_HOME}/../../logue-sdk/platform/nts-1_mkii/common/osc_api.h

exit 0
