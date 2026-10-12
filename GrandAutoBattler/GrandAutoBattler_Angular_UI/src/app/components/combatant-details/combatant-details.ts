import { Component, input, output } from '@angular/core';
import { CommonModule } from '@angular/common';
import type { Combatant } from '../../core/models/game.model';

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

  close(): void {
    this.closed.emit();
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
}
