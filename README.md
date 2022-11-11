# react-native-gzip

Fast gzip to compress strings for android & ios in React Native

## Installation

```sh
npm install react-native-gzip
```

## Usage

```js
import { deflate, inflate } from 'react-native-gzip';
const data = `hello world`;
const compressed = await deflate(data); // Returns a base64 string of compressed data
const decompressed = await inflate(compressed); // Decompress the data to original string value
```

## Contributing

See the [contributing guide](CONTRIBUTING.md) to learn how to contribute to the repository and the development workflow.

## License

MIT

---

Made with [create-react-native-library](https://github.com/callstack/react-native-builder-bob)
