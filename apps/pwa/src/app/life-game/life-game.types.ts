export type LifeGamePlayMode = 'play' | 'pause';

export interface LifeGameControl {
  playMode: LifeGamePlayMode;
}

export interface LifeGameConfig {
  width: number;
  height: number;
}
