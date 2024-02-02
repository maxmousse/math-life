import { ComponentFixture, TestBed } from '@angular/core/testing';
import { LifeGamePageComponent } from './life-game-page.component';

describe('LifeGamePageComponent', () => {
  let component: LifeGamePageComponent;
  let fixture: ComponentFixture<LifeGamePageComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [LifeGamePageComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(LifeGamePageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
