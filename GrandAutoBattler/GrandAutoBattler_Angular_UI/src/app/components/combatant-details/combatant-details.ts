import { Component, inject, input, output } from '@angular/core';
import { CommonModule } from '@angular/common';
import type { Combatant } from '../../core/models/game.model';
import { UnitImageService } from '../../core/services/unit-image.service';
import { PixelPicture } from '../pixel-picture/pixel-picture';

@Component({
  selector: 'app-combatant-details',
  standalone: true,
  imports: [CommonModule, PixelPicture],
  templateUrl: './combatant-details.html',
  styleUrl: './combatant-details.scss',
})
export class CombatantDetails {
  combatant = input.required<Combatant | null>();
  side = input.required<0 | 1>();
  closed = output<void>();
  imageService = inject(UnitImageService);

  close(): void {
    this.closed.emit();
  }

  ngOnInit() {}

  // Helper to get full display name
  getFullName(c: Combatant): string {
    const first = c.first_name ?? '';
    const last = c.last_name ?? '';
    return `${first} ${last}`.trim() || 'Unknown';
  }

  // Helper to get pretty unit type name
  getUnitType(c: Combatant): string {
    return c.unit_type_pretty_name || c.unit_type_name;
  }

  getUnitHealthBonus(c: Combatant): number {
    return (c.parameters.health-4) * 5;
  }

  // getCombatantPng = (combatant: Combatant, side: 0 | 1): string => {
  //   this.imageServicePng.updateConfig({ basePath: '/assets/images/128px', baseExtension: '.png' });
  //   return this.imageServicePng.getCombatantImagePath(combatant.unit_type_name, side);
  // };

  getCombatantPng = (c: Combatant, side: 0 | 1): string => {
    console.log('unit for png', c.unit_type_name);
    console.log('side for png', side);
    return this.imageService.getCombatantImagePath(c.unit_type_name, side, {
      basePath: '/assets/images/128px',
      baseExtension: '.png',
    });
  };

  // getCombatantGif = (combatant: Combatant, side: 0 | 1): string => {
  //   this.imageServiceGif.updateConfig({ basePath: '/assets/images/gif', baseExtension: '.gif' });
  //   return this.imageServiceGif.getCombatantImagePath(combatant.unit_type_name, side);
  // };
  getCombatantGif = (c: Combatant, side: 0 | 1): string => {
    console.log('unit for gif', c.unit_type_name);
    console.log('side for gif', side);

    return this.imageService.getCombatantImagePath(c.unit_type_name, side, {
      basePath: '/assets/images/gif',
      baseExtension: '.gif',
    });
  };
}
