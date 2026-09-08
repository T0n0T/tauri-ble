type RealtimeCommand = () => Promise<void>;

let realtimeCommandQueue = Promise.resolve();

export function enqueueRealtimeCommand(
  command: RealtimeCommand,
): Promise<void> {
  const next = realtimeCommandQueue.then(command, command);
  realtimeCommandQueue = next.catch(() => {});
  return next;
}
