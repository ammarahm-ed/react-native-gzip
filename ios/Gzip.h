
#ifdef RCT_NEW_ARCH_ENABLED
#import "RNGzipSpec.h"

@interface Gzip : NSObject <NativeGzipSpec>
#else
#import <React/RCTBridgeModule.h>

@interface Gzip : NSObject <RCTBridgeModule>
#endif

@end
