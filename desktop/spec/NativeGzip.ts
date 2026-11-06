import type { TurboModule } from 'react-native';
import { TurboModuleRegistry } from 'react-native';

export interface Spec extends TurboModule {
  inflate(data: string): Promise<string>;
  deflate(data: string): Promise<string>;
}

export default TurboModuleRegistry.getEnforcing<Spec>('Gzip');
