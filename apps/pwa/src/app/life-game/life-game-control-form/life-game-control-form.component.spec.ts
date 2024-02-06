import { ComponentFixture, TestBed } from '@angular/core/testing';
import { LifeGameFormComponent } from './life-game-control-form.component';

describe('LifeGameFormComponent', () => {
  let component: LifeGameFormComponent;
  let fixture: ComponentFixture<LifeGameFormComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [LifeGameFormComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(LifeGameFormComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
