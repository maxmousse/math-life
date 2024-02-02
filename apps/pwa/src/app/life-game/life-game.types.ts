export type LifeGamePlayMode = 'play' | 'pause';

export interface LifeGameControl {
  playMode: LifeGamePlayMode;
}

export interface LifeGameInitialState {
  width: number;
  heigh: number;
  initState: [number, number][];
}
