import { Component, inject, OnInit, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MockDataService } from '../../core/services/mock-data.service';
import { Unit } from '../../core/models/model';

@Component({
  selector: 'app-unit-list',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './unit-list.html',
  styleUrl: './unit-list.scss',
})
export class UnitList implements OnInit {
  private mockDataService = inject(MockDataService);

  units = signal<Unit[]>([]);
  factions = signal<string[]>([]);

  ngOnInit(): void {
    this.mockDataService.getFactions().subscribe(f => this.factions.set(f));
    this.mockDataService.getUnits('mock-id').subscribe(u => this.units.set(u));
  }

  getUnitsByFaction(faction: string): Unit[] {
    return this.units().filter(u => u.faction === faction);
  }
}
