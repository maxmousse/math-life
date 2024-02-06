import { ComponentFixture, TestBed } from '@angular/core/testing';
import { LifeGameConfigFormComponent } from './life-game-config-form.component';

describe('LifeGameConfigFormComponent', () => {
  let component: LifeGameConfigFormComponent;
  let fixture: ComponentFixture<LifeGameConfigFormComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [LifeGameConfigFormComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(LifeGameConfigFormComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
