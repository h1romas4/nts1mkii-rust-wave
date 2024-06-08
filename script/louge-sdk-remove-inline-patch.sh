#!/bin/bash

# remove static inline __attribute__ from all util files
pushd "$(dirname "$0")/../components/logue-sdk/platform/nts-1_mkii/common/utils"
ls | while read file
do
    cat $file | grep -v 'static inline __attribute__' > $file.tmp
    mv $file.tmp $file
done
popd

# remove static fast_inline
sed -i 's/static fast_inline//g' components/logue-sdk/platform/nts-1_mkii/common/osc_api.h
sed -i 's/static fast_inline//g' components/logue-sdk/platform/nts-1_mkii/common/fx_api.h
sed -i 's/static inline __attribute__((always_inline, optimize("Ofast")))//g' components/logue-sdk/platform/nts-1_mkii/common/fx_api.h
sed -i 's/static inline __attribute__((always_inline, optimize("Ofast")))//g' components/logue-sdk/platform/nts-1_mkii/common/osc_api.h

# TODO: remove powf
sed -i '/float dbampf(const float db) {/,/}/d' components/logue-sdk/platform/nts-1_mkii/common/utils/float_math.h
# TODO: remove log10f
sed -i '/float ampdbf(const float amp) {/,/}/d' components/logue-sdk/platform/nts-1_mkii/common/utils/float_math.h

exit 0
