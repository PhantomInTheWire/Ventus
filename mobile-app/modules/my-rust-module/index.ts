import {
  NativeModulesProxy,
  EventEmitter,
  Subscription,
} from "expo-modules-core";

// Import the native module. On web, it will be resolved to MyRustModule.web.ts
// and on native platforms to MyRustModule.ts
import MyRustModule from "./src/MyRustModule";
import MyRustModuleView from "./src/MyRustModuleView";
import {
  ChangeEventPayload,
  MyRustModuleViewProps,
} from "./src/MyRustModule.types";

// Get the native constant value.
export const PI = MyRustModule.PI;

export function hello(): string {
  return MyRustModule.hello();
}
// export async function rustAdd(a: number, b: number): Promise<number> {
//   return await MyRustModule.rustAdd(a, b);
// }

export async function ftpSync(
  host: string,
  port: number,
  local_dir: string,
  remote_dir: string
) {
  MyRustModule.ftpSync(host, port, local_dir, remote_dir);
}

export async function setValueAsync(value: string) {
  return await MyRustModule.setValueAsync(value);
}

const emitter = new EventEmitter(
  MyRustModule ?? NativeModulesProxy.MyRustModule
);

export function addChangeListener(
  listener: (event: ChangeEventPayload) => void
): Subscription {
  return emitter.addListener<ChangeEventPayload>("onChange", listener);
}

export { MyRustModuleView, MyRustModuleViewProps, ChangeEventPayload };
