/* ========================================================================
 * Copyright (c) 2005-2021 The OPC Foundation, Inc. All rights reserved.
 *
 * OPC Foundation MIT License 1.00
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation
 * files (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use,
 * copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following
 * conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
 * OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
 * HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 * WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * The complete license agreement can be found here:
 * http://opcfoundation.org/License/MIT/1.00/
 * ======================================================================*/

using System;
using System.Collections.Generic;
using System.Text;
using System.Xml;
using System.Runtime.Serialization;
using Opc.Ua;

namespace MyModel
{
    #region temperatureSensorState Class
    #if (!OPCUA_EXCLUDE_temperatureSensorState)
    /// <summary>
    /// Stores an instance of the temperatureSensor ObjectType.
    /// </summary>
    /// <exclude />
    [System.CodeDom.Compiler.GeneratedCodeAttribute("Opc.Ua.ModelCompiler", "1.0.0.0")]
    public partial class temperatureSensorState : BaseObjectState
    {
        #region Constructors
        /// <summary>
        /// Initializes the type with its default attribute values.
        /// </summary>
        public temperatureSensorState(NodeState parent) : base(parent)
        {
        }

        /// <summary>
        /// Returns the id of the default type definition node for the instance.
        /// </summary>
        protected override NodeId GetDefaultTypeDefinitionId(NamespaceTable namespaceUris)
        {
            return Opc.Ua.NodeId.Create(MyModel.ObjectTypes.temperatureSensor, MyModel.Namespaces.MyModel, namespaceUris);
        }

        #if (!OPCUA_EXCLUDE_InitializationStrings)
        /// <summary>
        /// Initializes the instance.
        /// </summary>
        protected override void Initialize(ISystemContext context)
        {
            base.Initialize(context);
            Initialize(context, InitializationString);
            InitializeOptionalChildren(context);
        }

        /// <summary>
        /// Initializes the instance with a node.
        /// </summary>
        protected override void Initialize(ISystemContext context, NodeState source)
        {
            InitializeOptionalChildren(context);
            base.Initialize(context, source);
        }

        /// <summary>
        /// Initializes the any option children defined for the instance.
        /// </summary>
        protected override void InitializeOptionalChildren(ISystemContext context)
        {
            base.InitializeOptionalChildren(context);
        }

        #region Initialization String
        private const string InitializationString =
           "AQAAACUAAABodHRwOi8vd3d3Lm9wY2ZvdW5kYXRpb24ub3JnL015TW9kZWwv/////wRggAIBAAAAAQAZ" +
           "AAAAdGVtcGVyYXR1cmVTZW5zb3JJbnN0YW5jZQEBAgABAQIAAgAAAP////8CAAAAFWCJCgIAAAABAAoA" +
           "AABzZW5zb3JOYW1lAQEEAAAuAEQEAAAAAAz/////AQH/////AAAAABVgiQoCAAAAAQARAAAAdGVtcGVy" +
           "YXR1cmVWYWx1ZUMBAQUAAC8APwUAAAABAQEA/////wEB/////wAAAAA=";
        #endregion
        #endif
        #endregion

        #region Public Properties
        /// <remarks />
        public PropertyState<string> sensorName
        {
            get
            {
                return m_sensorName;
            }

            set
            {
                if (!Object.ReferenceEquals(m_sensorName, value))
                {
                    ChangeMasks |= NodeStateChangeMasks.Children;
                }

                m_sensorName = value;
            }
        }

        /// <remarks />
        public BaseDataVariableState<temperature_reading> temperatureValueC
        {
            get
            {
                return m_temperatureValueC;
            }

            set
            {
                if (!Object.ReferenceEquals(m_temperatureValueC, value))
                {
                    ChangeMasks |= NodeStateChangeMasks.Children;
                }

                m_temperatureValueC = value;
            }
        }
        #endregion

        #region Overridden Methods
        /// <summary>
        /// Populates a list with the children that belong to the node.
        /// </summary>
        /// <param name="context">The context for the system being accessed.</param>
        /// <param name="children">The list of children to populate.</param>
        public override void GetChildren(
            ISystemContext context,
            IList<BaseInstanceState> children)
        {
            if (m_sensorName != null)
            {
                children.Add(m_sensorName);
            }

            if (m_temperatureValueC != null)
            {
                children.Add(m_temperatureValueC);
            }

            base.GetChildren(context, children);
        }

        /// <summary>
        /// Finds the child with the specified browse name.
        /// </summary>
        protected override BaseInstanceState FindChild(
            ISystemContext context,
            QualifiedName browseName,
            bool createOrReplace,
            BaseInstanceState replacement)
        {
            if (QualifiedName.IsNull(browseName))
            {
                return null;
            }

            BaseInstanceState instance = null;

            switch (browseName.Name)
            {
                case MyModel.BrowseNames.sensorName:
                {
                    if (createOrReplace)
                    {
                        if (sensorName == null)
                        {
                            if (replacement == null)
                            {
                                sensorName = new PropertyState<string>(this);
                            }
                            else
                            {
                                sensorName = (PropertyState<string>)replacement;
                            }
                        }
                    }

                    instance = sensorName;
                    break;
                }

                case MyModel.BrowseNames.temperatureValueC:
                {
                    if (createOrReplace)
                    {
                        if (temperatureValueC == null)
                        {
                            if (replacement == null)
                            {
                                temperatureValueC = new BaseDataVariableState<temperature_reading>(this);
                            }
                            else
                            {
                                temperatureValueC = (BaseDataVariableState<temperature_reading>)replacement;
                            }
                        }
                    }

                    instance = temperatureValueC;
                    break;
                }
            }

            if (instance != null)
            {
                return instance;
            }

            return base.FindChild(context, browseName, createOrReplace, replacement);
        }
        #endregion

        #region Private Fields
        private PropertyState<string> m_sensorName;
        private BaseDataVariableState<temperature_reading> m_temperatureValueC;
        #endregion
    }
    #endif
    #endregion
}