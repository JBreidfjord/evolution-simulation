import { Creature, Food } from '../wasm/simulation';

export interface World {
  creatures: Creature[];
  foods: Food[];
}