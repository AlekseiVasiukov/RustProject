import { Injectable, inject, signal, computed } from '@angular/core';
import { GameSessionService } from './game-session.service';

export interface ImageConfig {
  //Path to assets
  basePath: string;
  baseExtension: string;
  //Size demanded 64 or 128 px
  // size: string;
}

const DEFAULT_CONFIG: ImageConfig = {
  basePath: '/assets/images/64px', //or 127px, or gif // size: '64',
  baseExtension: '.png', //.gif
};

@Injectable({
  providedIn: 'root',
})
export class UnitImageService {
  private session = inject(GameSessionService);

  private config = signal<ImageConfig>({ ...DEFAULT_CONFIG });

  updateConfig(partial: Partial<ImageConfig>) {
    this.config.update((current) => ({ ...current, ...partial }));
  }

  //generating path for particular image
  private getImagePath(faction: string, unitTypeName: string): string {
    const cfg = this.config();
    return `${cfg.basePath}/${faction.toLowerCase()}-${unitTypeName.toLowerCase()}${cfg.baseExtension}`;
    // return `${cfg.basePath}/${cfg.size}/${faction.toLowerCase()}-${unitTypeName.toLowerCase()}.png`;
  }

  // getCombatantImagePath(combatantUnitTypeName: string, side: 0 | 1): string {
  //   const factions = this.session.availableFactions();
  //   const factionIndex =
  //     side === 0 ? this.session.ourFactionIndex() : this.session.opponentFactionIndex();
  //   const faction = factions[factionIndex] ?? 'unknown';
  //   return this.getImagePath(faction, combatantUnitTypeName);
  // }

  getCombatantImagePath(
    combatantUnitTypeName: string,
    side: 0 | 1,
    config?: Partial<ImageConfig>,
  ): string {
    const cfg = config ? { ...this.config(), ...config } : this.config();
    const factions = this.session.availableFactions();
    const factionIndex =
      side === 0 ? this.session.ourFactionIndex() : this.session.opponentFactionIndex();
    const faction = factions[factionIndex] ?? 'unknown';
    return `${cfg.basePath}/${faction.toLowerCase()}-${combatantUnitTypeName.toLowerCase()}${cfg.baseExtension}`;
  }

  // hasImage(faction, unitTypeName: string): boolean{
  //   return faction!=='unknown' && unitTypeName.length > 0;
  // }
}
