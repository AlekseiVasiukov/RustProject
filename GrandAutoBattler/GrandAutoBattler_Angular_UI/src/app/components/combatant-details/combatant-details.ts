import { Component, inject, input, output } from '@angular/core';
import { CommonModule } from '@angular/common';
import type { Combatant } from '../../core/models/game.model';
import { UnitImageService } from '../../core/services/unit-image.service';

@Component({
  selector: 'app-combatant-details',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './combatant-details.html',
  styleUrl: './combatant-details.scss',
})
export class CombatantDetails {
  combatant = input.required<Combatant | null>();
  closed = output<void>();
  imageServicePng = inject(UnitImageService);
  imageServiceGif = inject(UnitImageService);

  close(): void {
    this.closed.emit();
  }

  ngOnInit() {
    this.imageServicePng.updateConfig({ basePath: '/assets/images/128px' });
    this.imageServiceGif.updateConfig({ basePath: '/assets/images/gif', baseExtension: '.gif' });
  }

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

  getCombatantPng = (combatant: Combatant, side: 0 | 1): string => {
    return this.imageServicePng.getCombatantImagePath(combatant.unit_type_name, side);
  };

  getCombatantGif = (combatant: Combatant, side: 0 | 1): string => {
    return this.imageServiceGif.getCombatantImagePath(combatant.unit_type_name, side);
  };
}
