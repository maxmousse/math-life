import { ComponentFixture, TestBed } from '@angular/core/testing';
import { LeniaComponent } from './lenia.component';

describe('LeniaComponent', () => {
  let component: LeniaComponent;
  let fixture: ComponentFixture<LeniaComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [LeniaComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(LeniaComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
