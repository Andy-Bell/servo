/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use AnimationState;
use DocumentState;
use IFrameLoadInfo;
use MouseButton;
use MouseEventType;
use MozBrowserEvent;
use canvas_traits::CanvasMsg;
use euclid::point::Point2D;
use euclid::size::Size2D;
use gfx_traits::LayerId;
use ipc_channel::ipc::IpcSender;
use msg::constellation_msg::{Key, KeyModifiers, KeyState, LoadData};
use msg::constellation_msg::{NavigationDirection, PipelineId, SubpageId};
use offscreen_gl_context::{GLContextAttributes, GLLimits};
use style_traits::cursor::Cursor;
use style_traits::viewport::ViewportConstraints;
use url::Url;

/// Messages from the layout to the constellation.
#[derive(Deserialize, Serialize)]
pub enum LayoutMsg {
    /// Indicates whether this pipeline is currently running animations.
    ChangeRunningAnimationsState(PipelineId, AnimationState),
    /// Requests that the constellation inform the compositor of the a cursor change.
    SetCursor(Cursor),
    /// Notifies the constellation that the viewport has been constrained in some manner
    ViewportConstrained(PipelineId, ViewportConstraints),
}

/// Whether a DOM event was prevented by web content
#[derive(Deserialize, Serialize)]
pub enum EventResult {
    /// Allowed by web content
    DefaultAllowed,
    /// Prevented by web content
    DefaultPrevented,
}

/// A log entry reported to the constellation
/// We don't report all log entries, just serious ones.
/// We need a separate type for this because LogLevel isn't serializable.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LogEntry {
    /// Panic, with a reason and backtrace
    Panic(String, String),
    /// Error, with a reason
    Error(String),
    /// warning, with a reason
    Warn(String)
}

/// Messages from the script to the constellation.
#[derive(Deserialize, Serialize)]
pub enum ScriptMsg {
    /// Indicates whether this pipeline is currently running animations.
    ChangeRunningAnimationsState(PipelineId, AnimationState),
    /// Requests that a new 2D canvas thread be created. (This is done in the constellation because
    /// 2D canvases may use the GPU and we don't want to give untrusted content access to the GPU.)
    CreateCanvasPaintThread(Size2D<i32>, IpcSender<IpcSender<CanvasMsg>>),
    /// Requests that a new WebGL thread be created. (This is done in the constellation because
    /// WebGL uses the GPU and we don't want to give untrusted content access to the GPU.)
    CreateWebGLPaintThread(Size2D<i32>,
                           GLContextAttributes,
                           IpcSender<Result<(IpcSender<CanvasMsg>, GLLimits), String>>),
    /// Dispatched after the DOM load event has fired on a document
    /// Causes a `load` event to be dispatched to any enclosing frame context element
    /// for the given pipeline.
    DOMLoad(PipelineId),
    /// Notifies the constellation that this frame has received focus.
    Focus(PipelineId),
    /// Re-send a mouse button event that was sent to the parent window.
    ForwardMouseButtonEvent(PipelineId, MouseEventType, MouseButton, Point2D<f32>),
    /// Re-send a mouse move event that was sent to the parent window.
    ForwardMouseMoveEvent(PipelineId, Point2D<f32>),
    /// Requests that the constellation retrieve the current contents of the clipboard
    GetClipboardContents(IpcSender<String>),
    /// <head> tag finished parsing
    HeadParsed,
    /// All pending loads are complete.
    LoadComplete(PipelineId),
    /// A new load has been requested.
    LoadUrl(PipelineId, LoadData),
    /// Dispatch a mozbrowser event to a given iframe. Only available in experimental mode.
    MozBrowserEvent(PipelineId, SubpageId, MozBrowserEvent),
    /// HTMLIFrameElement Forward or Back navigation.
    Navigate(Option<(PipelineId, SubpageId)>, NavigationDirection),
    /// Favicon detected
    NewFavicon(Url),
    /// Status message to be displayed in the chrome, eg. a link URL on mouseover.
    NodeStatus(Option<String>),
    /// Notification that this iframe should be removed.
    RemoveIFrame(PipelineId, Option<IpcSender<()>>),
    /// Change pipeline visibility
    SetVisible(PipelineId, bool),
    /// Notifies constellation that an iframe's visibility has been changed.
    VisibilityChangeComplete(PipelineId, bool),
    /// A load has been requested in an IFrame.
    ScriptLoadedURLInIFrame(IFrameLoadInfo),
    /// Requests that the constellation set the contents of the clipboard
    SetClipboardContents(String),
    /// Mark a new document as active
    ActivateDocument(PipelineId),
    /// Set the document state for a pipeline (used by screenshot / reftests)
    SetDocumentState(PipelineId, DocumentState),
    /// Update the pipeline Url, which can change after redirections.
    SetFinalUrl(PipelineId, Url),
    /// Check if an alert dialog box should be presented
    Alert(PipelineId, String, IpcSender<bool>),
    /// Scroll a page in a window
    ScrollFragmentPoint(PipelineId, LayerId, Point2D<f32>, bool),
    /// Set title of current page
    /// https://html.spec.whatwg.org/multipage/#document.title
    SetTitle(PipelineId, Option<String>),
    /// Send a key event
    SendKeyEvent(Option<char>, Key, KeyState, KeyModifiers),
    /// Get Window Informations size and position
    GetClientWindow(IpcSender<(Size2D<u32>, Point2D<i32>)>),
    /// Move the window to a point
    MoveTo(Point2D<i32>),
    /// Resize the window to size
    ResizeTo(Size2D<u32>),
    /// Script has handled a touch event, and either prevented or allowed default actions.
    TouchEventProcessed(EventResult),
    /// Get Scroll Offset
    GetScrollOffset(PipelineId, LayerId, IpcSender<Point2D<f32>>),
    /// A log entry, with the pipeline id and thread name
    LogEntry(Option<PipelineId>, Option<String>, LogEntry),
    /// Notifies the constellation that this pipeline has exited.
    PipelineExited(PipelineId),
    /// Requests that the compositor shut down.
    Exit,
}
