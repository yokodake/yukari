use std::cmp::max;

use druid::{
    widget::{Container, Flex, Label, TextBox},
    Color, KeyOrValue, PaintCtx, Point, RenderContext,
};
use druid::{
    AppLauncher, BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PlatformError, Size, UpdateCtx, Widget, WindowDesc,
};

fn build_ui() -> impl Widget<()> {
    Container::new(
        FooterView::new(Label::new("body"), Label::new("footer"))
            .fill_body(true)
            .border(druid::Color::RED, 5.),
    )
}

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui).title("test")).launch(())
}

use druid::{Data, WidgetPod};

pub struct BorderStyle {
    pub width: KeyOrValue<f64>,
    pub color: KeyOrValue<Color>,
}

pub struct FooterView<T> {
    body: WidgetPod<T, Box<dyn Widget<T>>>,
    fill_body: bool,
    footer: WidgetPod<T, Box<dyn Widget<T>>>,
    border: Option<BorderStyle>,
    footer_size: Option<Size>,
}
impl<T: Data> FooterView<T> {
    pub fn new(body: impl Widget<T> + 'static, footer: impl Widget<T> + 'static) -> Self {
        Self {
            body: WidgetPod::new(body).boxed(),
            fill_body: false,
            footer: WidgetPod::new(footer).boxed(),
            border: None,
            footer_size: None,
        }
    }
    pub fn set_fill_body(&mut self, fill: bool) {
        self.fill_body = fill;
    }
    pub fn fill_body(mut self, fill: bool) -> Self {
        self.set_fill_body(fill);
        self
    }
    pub fn set_border(
        &mut self,
        color: impl Into<KeyOrValue<Color>>,
        width: impl Into<KeyOrValue<f64>>,
    ) {
        self.border = Some(BorderStyle {
            color: color.into(),
            width: width.into(),
        });
    }
    pub fn border(
        mut self,
        color: impl Into<KeyOrValue<Color>>,
        width: impl Into<KeyOrValue<f64>>,
    ) -> Self {
        self.set_border(color, width);
        self
    }
}
impl<T: Data> Widget<T> for FooterView<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.body.event(ctx, event, data, env);
        self.footer.event(ctx, event, data, env);
    }
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.body.lifecycle(ctx, event, data, env);
        self.footer.lifecycle(ctx, event, data, env);
    }
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.body.update(ctx, data, env);
        self.footer.update(ctx, data, env);
    }
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("footerView");
        // footer layout
        let border_width = self.border.as_ref().map_or(0., |x| x.width.resolve(env));
        let fbc = bc.loosen();
        let fsize = self.footer.layout(ctx, &fbc, data, env);
        self.footer_size = Some(fsize);

        // body layout
        let bbc = if !self.fill_body {
            bc.loosen().shrink((0.0, fsize.height + border_width))
        } else {
            bc.shrink((0., fsize.height + border_width))
        };
        let bsize = self.body.layout(ctx, &bbc, data, env);
        let origin = Point::new(0.0, 0.0);
        self.body.set_origin(ctx, data, env, origin);

        // footer origin
        self.footer
            .set_origin(ctx, data, env, Point::new(0.0, bsize.height + border_width));

        let my_size = Size::new(
            fsize.width.max(bsize.width),
            bsize.height + fsize.height + border_width,
        );

        let child_paint_rect = self.footer.paint_rect().union(self.body.paint_rect());
        let my_bounds = druid::Rect::ZERO.with_size(my_size);
        let my_insets = child_paint_rect - my_bounds;
        ctx.set_paint_insets(my_insets);
        print!("{} ", my_size);
        my_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        if let Some(fsize) = &self.footer_size {
            if let Some(border) = &self.border {
                let Size { width, height } = ctx.size();
                let height = height - fsize.height;
                let border_width = border.width.resolve(env);
                let line =
                    druid::kurbo::Line::new(Point::new(0., height), Point::new(width, height));
                ctx.stroke(line, &border.color.resolve(env), border_width);
            }
        };

        println!("=> {}", ctx.size());

        self.body.paint(ctx, data, env);
        self.footer.paint(ctx, data, env);
    }
}
