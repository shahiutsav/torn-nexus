export interface UserData {
  profile: Profile;
  bars: Bars;
  cooldowns: Cooldowns;
}

export interface Profile {
  id: number;
  name: string;
  level: number;
  rank: string;
  title: string;
  age: number;
  signed_up: number;
  faction_id: number;
  honor_id: number;
  property: Property;
  donator_status: string;
  image: string;
  gender: string;
  revivable: boolean;
  role: string;
  status: Status;
  spouse: Spouse;
  awards: number;
  friends: number;
  enemies: number;
  forum_posts: number;
  karma: number;
  last_action: LastAction;
  life: Life;
}

export interface Property {
  id: number;
  name: string;
}

export interface Status {
  description: string;
  details: any;
  state: string;
  color: string;
  until: any;
}

export interface Spouse {
  id: number;
  name: string;
  status: string;
  days_married: number;
}

export interface LastAction {
  status: string;
  timestamp: number;
  relative: string;
}

export interface Life {
  current: number;
  maximum: number;
}

export interface Bars {
  energy: Bar;
  nerve: Bar;
  happy: Bar;
  life: Bar;
  chain: Chain;
}

export interface Bar {
  current: number;
  maximum: number;
  increment: number;
  interval: number;
  tick_time: number;
  full_time: number;
}

export interface Chain {
  id: number;
  current: number;
  max: number;
  timeout: number;
  modifier: number;
  cooldown: number;
  start: number;
  end: number;
}

export interface Cooldowns {
  drug: number;
  medical: number;
  booster: number;
}
