import * as React from 'react';
import { decode } from 'base64-arraybuffer';
import { decompressSync, strFromU8 } from 'fflate';
import { Button, StyleSheet, View } from 'react-native';
import { deflate, inflate } from 'react-native-gzip';
global.Buffer = require('buffer').Buffer;

export const decompress = (compressed: string) => {
  return strFromU8(decompressSync(new Uint8Array(decode(compressed))));
};

const data = `hello world`;
export default function App() {
  return (
    <View style={styles.container}>
      <Button
        title="Test Compression"
        onPress={async () => {
          const compressed = await deflate(data);
          const decompressed = await inflate(compressed);
          console.log(compressed, decompressed);
        }}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
  },
  box: {
    width: 60,
    height: 60,
    marginVertical: 20,
  },
});
