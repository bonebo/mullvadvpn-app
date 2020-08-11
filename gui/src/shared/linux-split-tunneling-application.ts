type Warning = 'launches-in-existing-process';

export default interface ISplitTunnelingApplication {
  absolutepath: string;
  name: string;
  exec?: string;
  icon?: string;
  warning?: Warning;
}
