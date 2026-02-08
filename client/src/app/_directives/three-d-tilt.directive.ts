import { Directive, ElementRef, HostListener, Renderer2, Input } from '@angular/core';

@Directive({
    selector: '[appThreeDTilt]',
    standalone: true
})
export class ThreeDTiltDirective {
    @Input('appThreeDTilt') intensity: number = 40; // Alias input to selector name

    constructor(private el: ElementRef, private renderer: Renderer2) {
        console.log('3D Tilt Directive Initialized on:', this.el.nativeElement);
        this.renderer.setStyle(this.el.nativeElement, 'transform-style', 'preserve-3d');
    }

    @HostListener('mousemove', ['$event'])
    onMouseMove(e: MouseEvent) {
        const rect = this.el.nativeElement.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;

        // Normalized 0 to 1
        const px = x / rect.width;
        const py = y / rect.height;

        // Rotation (-0.5 to 0.5) * Intensity
        const rotateY = (px - 0.5) * this.intensity;
        const rotateX = (py - 0.5) * -this.intensity;

        // Apply Transform with better perspective
        const transform = `perspective(800px) rotateX(${rotateX}deg) rotateY(${rotateY}deg) scale(1.08)`;
        this.renderer.setStyle(this.el.nativeElement, 'transform', transform);

        // Update CSS variables for advanced effects
        const el = this.el.nativeElement;
        el.style.setProperty('--mx', `${px * 100}%`);
        el.style.setProperty('--my', `${py * 100}%`);
        el.style.setProperty('--rx', `${rotateX}deg`);
        el.style.setProperty('--ry', `${rotateY}deg`);
        el.style.setProperty('--opacity', '1');

        // Glare legacy vars
        el.style.setProperty('--pointer-x', `${px * 100}%`);
        el.style.setProperty('--pointer-y', `${py * 100}%`);
        el.style.setProperty('--card-opacity', '1');
    }

    @HostListener('mouseleave')
    onMouseLeave() {
        this.renderer.setStyle(this.el.nativeElement, 'transition', 'transform 0.5s cubic-bezier(0.2, 1, 0.2, 1)');
        this.renderer.setStyle(this.el.nativeElement, 'transform', 'perspective(800px) rotateX(0deg) rotateY(0deg) scale(1)');
        this.el.nativeElement.style.setProperty('--opacity', '0');
        this.el.nativeElement.style.setProperty('--card-opacity', '0');
    }

    @HostListener('mouseenter')
    onMouseEnter() {
        this.renderer.setStyle(this.el.nativeElement, 'transition', 'none');
    }
}
