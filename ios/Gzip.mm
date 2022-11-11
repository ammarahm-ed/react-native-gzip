#import "Gzip.h"
#import "Base64/MF_Base64Additions.h"
#import "GZIP/NSData+GZIP.h"

@implementation Gzip
RCT_EXPORT_MODULE()

RCT_REMAP_METHOD(inflate,
                 base64:(NSString *)base64
                 withResolver:(RCTPromiseResolveBlock)resolve
                 withRejecter:(RCTPromiseRejectBlock)reject)
{
  NSData * _data = [NSData dataWithBase64String:base64];
  
  resolve([[NSString alloc] initWithData:[_data gunzippedData] encoding:NSUTF8StringEncoding]);
}

RCT_REMAP_METHOD(deflate,
                 data:(NSString *)data
                 withResolver:(RCTPromiseResolveBlock)resolve
                 withRejecter:(RCTPromiseRejectBlock)reject)
{
  NSData * _data = [data dataUsingEncoding: NSUTF8StringEncoding];
  resolve([[_data gzippedData] base64String]);
}

// Don't compile this code when we build for the old architecture.
#ifdef RCT_NEW_ARCH_ENABLED
- (std::shared_ptr<facebook::react::TurboModule>)getTurboModule:
    (const facebook::react::ObjCTurboModule::InitParams &)params
{
    return std::make_shared<facebook::react::NativeGzipSpecJSI>(params);
}
#endif

@end
